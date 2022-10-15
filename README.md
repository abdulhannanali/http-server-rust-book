# Rust HTTP Server 

## Demo

Following Demos are available

|Server|Link|Description|
|---|---|---|
|Multi Threaded Demo|https://multithreadrustserver-mcl6zwvdla-uc.a.run.app/|Demo Deployed On CloudRun|
|Single Threaded Demo|https://singlethreadedrustserver-mcl6zwvdla-uc.a.run.app/|Demo Deployed on CloudRun|

## Overview 

This is the demo built as a result of following the Rust Book's [Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html). The chapter provides implementation guide for two types of Web Servers.

- [Single Threaded Web Server](https://doc.rust-lang.org/book/ch20-01-single-threaded.html)
- [Multi Threaded Web Server](https://doc.rust-lang.org/book/ch20-02-multithreaded.html)


Both of these servers have been implemented their respective directories.


Although, the book has been closely followed. At some places I have made little changes to separate some of the logics in their respective modules. Further, I plan to add more features into this server as well, as a learning exercise. 

You can refer to Github Issues on what's remaining and what's currently in progress.


### Additional Features

#### Google Cloud Build
        
To set up the Continuous Deployment, I used Google Cloud Build. The Continuous Deployment Trigger is set up on `master` branch.

Triggers for both perform the following steps.

- Build the Docker Image
- Push the image to Google Artifacts Repository
- Deploy the image for the corresponding Cloud Run Service.

The one time triggers are setup using Manual commands available in [gbuild/scripts](./gbuild/scripts)

Github Repositories are first connected through Google Cloud UI since it's not available on command as of yet.

