**Simple Auth Module**
=====================

A lightweight, easy-to-use authentication module that provides simple username/password authentication and role-based access control.

**Features**

* **Username/Password Authentication**: Users can authenticate using their username and password.
* **Role-Based Access Control (RBAC)**: Assign roles to users and control access to resources based on those roles.
* **user Management**: Subscribe user
* **Provide role rules**: Provide role rules to restrict endpoint

**Getting Started**
-------------------

**API Endpoints**
-----------------

### Users

* `GET /users`: Retrieve a list of all users.
* `POST /users`: Create a new user account.
* `PUT /users/:id`: Update an existing user account.
* `DELETE /users/:id`: Delete a user account.

### Roles

Role is hard coded because I don't known if it's really necessary.

We have two app role :

- **SuperAdmin :** Allow all for developpers (As technical function)
- **Admin :** Allow a lot feature (All business function for customer)
- **Moderator :** Allow a lot feature for moderate the content
- **User :** Normal usage for basic user


#### Roles usage

If we want group management or premium access I think it's better to manage it on separate module with separate rules

### Authentication

* `POST /login`: Authenticate a user and return a JSON Web Token (JWT) token.

### AuthDataStore

#### Table

- ***auth_informations*** : Username Password association to verify login
  - oid : Users id
  - Username : String
  - Password : String
  - Created_at : DateTime
  - Last_password_edited_at : DateTime
  - roles : Role[]
  - connection_history: DateTime // TODO
  
 > Roles is on separated table because Password authentification is not the only way to authentificate in future
 > It's easier if we want add method or delete this method

**Security**
-------------

This module uses [PASETORS](https://github.com/brycx/pasetors?tab=readme-ov-file) for authentication. [PASETO](https://paseto.io/) tokens are securely signed with a secret key to prevent tampering or forgery.
