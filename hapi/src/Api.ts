import * as Hapi from "hapi";
import {getManager} from "typeorm";
import {Footballer} from "./entity/Footballer";

export async function create(request: Hapi.Request, h: Hapi.ResponseToolkit) {
    let footballerRepository = getManager().getRepository(Footballer);

    const footballer = <Footballer>request.payload;
    let savedFootballer = await footballerRepository.save(footballer).catch(logError);
    return h.response(footballer).code(201);
}

export async function search(request: Hapi.Request, h: Hapi.ResponseToolkit) {
    let footballerRepository = getManager().getRepository(Footballer);

    if (request.query.position === undefined) {
        return footballerRepository.find();
    }
    return footballerRepository.find({position: `${request.query.position}`}).catch(logError);
}

export async function findById(request: Hapi.Request, h: Hapi.ResponseToolkit) {
    let footballerRepository = getManager().getRepository(Footballer);

    let footballer = await footballerRepository.findOne(`${request.params.id}`).catch(logError);
    if (footballer === undefined) {
        return h.response({}).code(404);
    } else {
        return h.response(footballer).code(200);
    }
}

export async function deleteById(request: Hapi.Request, h: Hapi.ResponseToolkit) {
    let footballerRepository = getManager().getRepository(Footballer);

    await footballerRepository.findOne(`${request.params.id}`).then(footballer => {
        if (footballer !== undefined) {
            footballerRepository.remove([footballer]).catch(logError);
        }
    }).catch(logError);
    return h.response({}).code(204);
}

function logError(err) {
    console.log(err);
    return {};
}