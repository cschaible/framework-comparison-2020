package info.novatec;

import io.smallrye.mutiny.Multi;
import io.smallrye.mutiny.Uni;
import io.vertx.mutiny.pgclient.PgPool;
import org.jboss.resteasy.annotations.jaxrs.QueryParam;

import javax.inject.Inject;
import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;

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
  public Uni<Footballer> get(@PathParam("id") Long id) {
    return Footballer.findOne(client, id);
  }

  @POST
  public Uni<Footballer> create(Footballer footballer) {
    return Footballer.create(client, footballer);
  }

  @DELETE
  @Path("/{id}")
  public Uni<Boolean> delete(@PathParam("id") Long id) {
    return Footballer.delete(client, id);
  }
}
