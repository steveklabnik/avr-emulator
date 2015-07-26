# Building

**Requirements:** Install [boot2docker](http://boot2docker.io/) and make sure it's running

**Note:** All commands below assume you are working from this `./server` directory.

```
docker build -f rust-dockerfile -t realscouteng/rust-emulator .
docker build -f avr-dockerfile -t realscouteng/avr-emulator .
```

**Note:** If you are getting ssl errors when building, run `boot2docker restart`.

# Running Docker Locally

``` bash
# In a separate terminal session
boot2docker ssh -vnNTL 8000:localhost:8000
```

```bash
docker run -p 8000:8000 -t realscouteng/avr-emulator
```

# Updating the avr-emulator docker image

```bash
docker push realscouteng/avr-emulator
```

# Deployment

See [Joyent Docs](https://apidocs.joyent.com/docker/) for more info

```bash
bash sdc-docker-setup.sh https://us-east-1.api.joyent.com chris@realscout.com ~/.ssh/chris-triton
```

```bash
docker run -p 8000:8000 -d realscouteng/avr-emulator
```

**NB:** This will issue a new public ip address every time - I have a support ticket with Joyent asking how to keep a
static ip address.
