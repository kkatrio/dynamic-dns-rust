## Dynamic dns in Rust

##### Build image:
docker build -t dns .

##### Run with: 
docker run -d -v *absolute-path-to*/.env:/.env dns

##### .env: 
cp env.example .env
