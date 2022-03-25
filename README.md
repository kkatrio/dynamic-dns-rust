## Dynamic dns in Rust

##### Build image:
docker build -t dns .

##### Run with: 
docker run -d -v *absolute-path-to*/.env:/.env dns

##### .env: 
ACCESS_TOKEN=abcdef  
DOMAIN='domain.example.xyz'  
ZONE='zone_xyz'  
