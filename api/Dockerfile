FROM node:alpine AS builder

WORKDIR /app

COPY package.json pnpm-lock.yaml ./
RUN npm install -g pnpm && pnpm install --frozen-lockfile

COPY . .
RUN npm run build

FROM node:alpine

WORKDIR /app

COPY --from=builder /app/dist ./dist
COPY package.json pnpm-lock.yaml ./

RUN npm install -g pnpm && pnpm install --prod --frozen-lockfile

RUN addgroup --system appgroup && adduser --system --ingroup appgroup appuser
USER appuser

CMD ["npm", "run", "start:prod"]
