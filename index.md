## Script CMS 

Note: This is a work in progress and incomplete in many respects. 

A Headless CMS written entirely in Rust

### Architecture

Script is intended to be used as a headless CMS and makes no assumptions or assertions on the client technology or framework.
All data will be exposed as REST APIs.

#### Role based access control

JWTs in the Authorization Bearer header is used for RBAC. The verification of the JWT will be done against a jwks url specified in the JWKS_URL environment variable. (For development, HMAC + SHA256 with the key as 123456 is used). 

Paths

/admin/rbac is the only path that is enabled by default. This can be used to create new policies. 

/admin/site can be used to create new site. 

GET /site/:sitename can be used to read site details
PUT /site/:sitename can be used to update site details
DELETE /site/:sitename can be used to delete site and related policies


### Data Model

It is expected that a lot of uses of this CMS will be for supplying dynamic data and content to websites. Script is intended to be a multi tenant CMS with the site at the root of every tenant. A site can contain one or more collections. Collections hold groups of data elements that can be grouped to from one logical unit.

#### Site

A Site is a logical grouping of data elements that can be managed as one unit. A site is expected to provide dynamic data and content for a page on a website. It could also be used to provide content for mobile applications or other such entities. 


The data elements are of the following types

* Text
* Boolean
* Number
* Asset


