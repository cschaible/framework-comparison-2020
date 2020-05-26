package info.novatec;

import javax.transaction.Transactional;
import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import java.util.List;
import java.util.Optional;

@Path("/footballers")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class FootballerResource {

  @GET
  @Transactional
  public List<Footballer> search(@QueryParam("position") String position) {
    if (position == null) {
      return Footballer.findAll().list();
    } else {
      return Footballer.findByPosition(position).list();
    }
  }

  @GET
  @Path("/{id}")
  @Transactional
  public Optional<Footballer> get(@PathParam("id") Long id) {
    return Footballer.find("id", id).firstResultOptional();
  }

  @POST
  @Transactional
  public Footballer create(Footballer footballer) {
    footballer.persist();
    return footballer;
  }

  @DELETE
  @Path("/{id}")
  @Transactional
  public void delete(@PathParam("id") Long id) {
    Footballer.delete("id", id);
  }
}
