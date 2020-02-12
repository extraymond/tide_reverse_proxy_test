Test procedure:

1. launch the app with either ```cargo run --bin hyper|tide```
2. make sure it's accessible from localhost:8080/
3. follow the instruction for reverse proxy from the test/$PROXY folder
4. after starting reverse proxy up, run ```curl http://localhost:5000/api/```
