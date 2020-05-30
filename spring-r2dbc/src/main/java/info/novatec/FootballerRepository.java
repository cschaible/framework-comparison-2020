package info.novatec;

import org.reactivestreams.Publisher;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.r2dbc.core.DatabaseClient;
import org.springframework.data.relational.core.query.Criteria;
import org.springframework.stereotype.Component;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

@Component
public class FootballerRepository {

  @Autowired private DatabaseClient databaseClient;

  public Mono<Footballer> create(Publisher<Footballer> footballer) {
    return databaseClient
        .insert()
        .into(Footballer.class)
        .using(footballer)
        .map(
            (row, metadata) -> {
              Footballer createdFootballer = new Footballer();
              createdFootballer.setId(row.get("id", Long.class));
              createdFootballer.setFirstName(row.get("first_name", String.class));
              createdFootballer.setLastName(row.get("last_name", String.class));
              createdFootballer.setPosition(row.get("position", String.class));
              return createdFootballer;
            })
        .one();
  }

  public Flux<Footballer> findByPosition(String position) {
    return databaseClient
        .select()
        .from(Footballer.class)
        .matching(Criteria.where("position").is(position))
        .fetch()
        .all();
  }

  public Mono<Footballer> findById(Long id) {
    return databaseClient
        .select()
        .from(Footballer.class)
        .matching(Criteria.where("id").is(id))
        .fetch()
        .one();
  }

  public Flux<Footballer> findAll() {
    return databaseClient.select().from(Footballer.class).fetch().all();
  }

  public Mono<Void> deleteById(Long id) {
    return databaseClient
        .delete()
        .from(Footballer.class)
        .matching(Criteria.where("id").is(id))
        .then();
  }
}
