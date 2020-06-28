package info.novatec;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;

import javax.transaction.Transactional;
import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;
import java.util.List;
import java.util.Optional;

@Path("/footballers")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class FootballerResource {

  @GET
  public List<Footballer> search(@QueryParam("position") String position) {
    if (position == null) {
      return Footballer.findAll().list();
    } else {
      return Footballer.findByPosition(position).list();
    }
  }

  @GET
  @Path("/{id}")
  public Response get(@PathParam("id") Long id) {
    Optional<PanacheEntityBase> footballer = Footballer.find("id", id).firstResultOptional();
    if (footballer.isPresent()) {
      return Response.ok(footballer.get()).build();
    } else {
      return Response.status(Response.Status.NOT_FOUND).build();
    }
  }

  @POST
  @Transactional
  public Response create(Footballer footballer) {
    footballer.persist();
    return Response.status(Response.Status.CREATED).entity(footballer).build();
  }

  @DELETE
  @Path("/{id}")
  @Transactional
  public Response delete(@PathParam("id") Long id) {
    Footballer.delete("id", id);
    return Response.noContent().build();
  }
}
