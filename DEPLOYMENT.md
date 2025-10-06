# Deploying to fly.io

## Prerequisites

1. Install the Fly CLI: https://fly.io/docs/hands-on/install-flyctl/
2. Sign up for a fly.io account: `flyctl auth signup` or login: `flyctl auth login`

## Deployment Steps

### 1. Launch your app (first time only)

```bash
flyctl launch
```

This will:
- Detect your Dockerfile
- Ask you to choose an app name (or use the default: `rust-by-example`)
- Ask you to choose a region (choose one close to your users)
- Create a fly.toml configuration file (already created)
- NOT deploy immediately (you can deploy manually)

**Important**: When asked "Would you like to set up a Postgresql database?", say **NO**.
When asked "Would you like to deploy now?", say **NO** (we'll deploy in the next step).

### 2. Deploy your app

```bash
flyctl deploy
```

This will:
- Build your Docker image
- Push it to fly.io's registry
- Deploy it to your app
- Give you a URL like: https://rust-by-example.fly.dev

### 3. Check your app status

```bash
flyctl status
```

### 4. View logs

```bash
flyctl logs
```

### 5. Open your app in browser

```bash
flyctl open
```

## Configuration

### Port Configuration
- The app now binds to `0.0.0.0:8080` in production (or uses the `PORT` env variable)
- In development (local), it defaults to `0.0.0.0:8080` if no PORT is set

### Environment Detection
- The CSS file watcher is automatically disabled in production (when `FLY_APP_NAME` env var exists)
- In development, the file watcher runs to auto-regenerate CSS

## Updating Your App

After making changes:

```bash
flyctl deploy
```

## Scaling

To scale your app:

```bash
# Scale to 2 machines
flyctl scale count 2

# Scale machine resources
flyctl scale vm shared-cpu-1x --memory 512
```

## Environment Variables

To set environment variables:

```bash
flyctl secrets set MY_SECRET=value
```

## Custom Domain

To add a custom domain:

```bash
flyctl certs add yourdomain.com
```

Then add the DNS records shown in the output.

## Troubleshooting

### Check logs
```bash
flyctl logs
```

### SSH into your machine
```bash
flyctl ssh console
```

### Check machine status
```bash
flyctl status
```

### Monitor your app
```bash
flyctl dashboard
```

## Cost
- fly.io has a generous free tier
- You get 3 shared-cpu-1x VMs with 256MB RAM for free
- See: https://fly.io/docs/about/pricing/

