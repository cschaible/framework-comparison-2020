import * as Hapi from "hapi";

import * as DotEnv from "dotenv";

import {create, deleteById, findById, search} from "./Api"
import {ConnectionOptions, createConnection} from "typeorm";
import {SnakeNamingStrategy} from "typeorm-naming-strategies";

const host = "0.0.0.0";
const port = 8080;

const result = DotEnv.config()
if (result.error) {
    throw result.error
}

const config: ConnectionOptions = {
    type: 'postgres',
    host: process.env.TYPEORM_HOST,
    username: process.env.TYPEORM_USERNAME,
    password: process.env.TYPEORM_PASSWORD,
    database: process.env.TYPEORM_DATABASE,
    synchronize: process.env.TYPEORM_SYNC === 'true',
    logging: process.env.TYPEORM_LOGGING === 'true',
    migrationsRun: process.env.TYPEORM_MIGRATIONS_RUN === 'true',
    entities: [process.env.TYPEORM_ENTITIES_DIR],
    migrations: [process.env.TYPEORM_MIGRATIONS_DIR],
    namingStrategy: new SnakeNamingStrategy(),
    cli: {
        migrationsDir: 'src/migration',
    },
};

/** Configure pool */

createConnection(config).then(_connection => {
    console.log("Ready");
}).catch(err => console.log(err));

/** Configure server */
const server: Hapi.Server = new Hapi.Server({host, port});

// To log all errors use the following definition
//const server: Hapi.Server = new Hapi.Server({host, port, debug: { request: ['error'] } });

/** Register routes */

server.route({
    method: "POST",
    path: '/footballers',
    handler: create
});

server.route({
    method: "GET",
    path: '/footballers',
    handler: search
});

server.route({
    method: "GET",
    path: '/footballers/{id}',
    handler: findById
});

server.route({
    method: "DELETE",
    path: '/footballers/{id}',
    handler: deleteById
});

/** Start the server */

async function start() {
    try {
        await server.start();
    } catch (err) {
        console.log(err);
        process.exit(1);
    }
    console.log(`Server running @ ${server.info.uri}`);
}

start().catch(err => console.log(err));
