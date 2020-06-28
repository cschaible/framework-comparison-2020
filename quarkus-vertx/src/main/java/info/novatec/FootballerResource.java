package info.novatec;

import io.smallrye.mutiny.Multi;
import io.smallrye.mutiny.Uni;
import io.vertx.mutiny.pgclient.PgPool;
import org.jboss.resteasy.annotations.jaxrs.QueryParam;

import javax.inject.Inject;
import javax.transaction.Transactional;
import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;

@Path("/footballers")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class FootballerResource {

  @Inject PgPool client;

  @GET
  public Multi<Footballer> findAll(@QueryParam(value = "position") String position) {
    if (position == null) {
      return Footballer.findAll(client);
    } else {
      return Footballer.findByPosition(client, position);
    }
  }

  @GET
  @Path("/{id}")
  public Uni<Response> get(@PathParam("id") Long id) {
    return Footballer.findOne(client, id)
        .onItem()
        .apply(
            footballer -> {
              if (footballer == null) {
                return Response.status(Response.Status.NOT_FOUND).build();
              } else {
                return Response.ok(footballer).build();
              }
            });
  }

  @Transactional
  @POST
  public Uni<Response> create(Footballer footballer) {
    return Footballer.create(client, footballer)
        .onItem()
        .apply(
            savedFootballer ->
                Response.status(Response.Status.CREATED).entity(savedFootballer).build());
  }

  @Transactional
  @DELETE
  @Path("/{id}")
  public Uni<Response> delete(@PathParam("id") Long id) {
    return Footballer.delete(client, id)
        .onItem()
        .apply(deleted -> Response.noContent().entity(deleted).build());
  }
}
