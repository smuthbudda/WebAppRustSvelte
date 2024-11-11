# ./Dockerfile

# Use the alpine version that suits your needs, based
# on the Node version you want to use. If possible use
# the latest stable version (recommended for a newly
# created SvelteKit project).
FROM node:alpine

# Use whichever container directory you want as long
# as you make sure to use it consistently in further
# configuration. I'll just use '/app' here for simplicity.
WORKDIR '/app'

COPY package.json .
RUN npm install

COPY . .

# Using the --host 0.0.0.0 option (which is passed to Vite)
# was the only way I could get the dev/HMR mode to be
# accessible from outside the container.
CMD npm run dev -- --host 0.0.0.0
