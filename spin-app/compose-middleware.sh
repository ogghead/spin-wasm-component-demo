# Use WAC to compose the auth component with Spin APIs
wac plug ../../auth/target/wasm32-wasip1/release/auth.wasm --plug $1 -o $1
