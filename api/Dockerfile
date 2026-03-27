FROM node:alpine

COPY package.json pnpm-lock.yaml .

RUN npm install -g pnpm && pnpm install --frozen-lockfile

COPY . .

RUN npm run build

CMD ["npm", "run", "start:prod"]
