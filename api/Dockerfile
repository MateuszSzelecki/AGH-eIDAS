FROM node:alpine

COPY package.json .

RUN npm install --verbose

COPY . .

RUN npm run build

CMD ["npm", "run", "start:prod"]
