@run:
    wrangler dev

@setup:
    just install-worker-build

@create-wrangler-config:
    cp wrangler.example.toml wrangler.toml
    
@install-worker-build:
    cargo install worker-build
