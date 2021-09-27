use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{web::Data, Error, HttpMessage, HttpResponse};
use futures::future::{ok, Either, Ready};
use log::{debug, error, trace};

use crate::rbac::{utils, Authenticate, RbacParams};
use crate::AppData;
use crate::rbac::models::Identity;

impl<S, B> Transform<S> for Authenticate
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckTokenMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckTokenMiddleware { service })
    }
}
pub struct CheckTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service for CheckTokenMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {

        let app_data = req.app_data::<Data<AppData>>().unwrap();
        trace!("Request {:?}", req);

        //Check if path and method combination needs to be authenticated
        let path: String= req.path().to_string();
        debug!("Path requested {}", path);

        let method: String = req.method().to_string();
        debug!("Method requested {}", method);
        //let unauthorized: HttpResponse<B> = HttpResponse::Unauthorized().finish().into_body();

        let is_public_path = match &req.app_data::<Data<AppData>>() {
            Some(app_data) => {
                match &app_data.rbac.lock() {
                    Ok(rbac) => {
                        let public_paths: &Vec<String> = &rbac.public_paths;
                        debug!("Checking if {} is in {:?}", path, public_paths);
                        if (*public_paths).contains(&path){
                            true
                        } else {
                            false
                        }
                    },
                    Err(e)=> {
                        error!("Error getting lock on rbac policy");
                        false
                    }
                }
            }
            None => {
               debug!("Cannot get app data");
               false 
            }
        };

        debug!("Is a public path {}", is_public_path);
        if method == "GET" && is_public_path{
            debug!("Public path..will continue without token validation");
            let anonymous = Identity {
                user: "Anonymous".to_string(),
                roles: vec!("ANONYMOUS".to_string()),
            };
            req.extensions_mut().insert(anonymous);
            return Either::Left(self.service.call(req));
        }

        let identity = match utils::check_token(&req) {
            Err(auth_error) => {
                error!("Authentication error {:?}", auth_error);
                let u: actix_web::HttpResponse<B> =
                    HttpResponse::Unauthorized().json(auth_error).into_body();
                return Either::Right(ok(req.into_response(u)));
            }
            Ok(identity) => identity,
        };

        let rbac_params = RbacParams {
            method: method.to_string(),
            path: path.to_string(),
            rbac_role: identity.roles.to_vec(),
            rbac_user: identity.user.to_string(),
        };

        debug!("Rbac Params {:?}", rbac_params);

        match utils::check_rbac(rbac_params, app_data) {
            Ok(_) => {
                req.extensions_mut().insert(identity);
                Either::Left(self.service.call(req))
            }
            Err(auth_error) => {
                debug!("Authentication error {:?}", auth_error);
                let u: actix_web::HttpResponse<B> =
                    HttpResponse::Unauthorized().json(auth_error).into_body();
                Either::Right(ok(req.into_response(u)))
            }
        }
    }
}
