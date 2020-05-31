package info.novatec;

import io.smallrye.mutiny.Multi;
import io.smallrye.mutiny.Uni;
import io.vertx.mutiny.pgclient.PgPool;
import io.vertx.mutiny.sqlclient.Pool;
import io.vertx.mutiny.sqlclient.Row;
import io.vertx.mutiny.sqlclient.RowSet;
import io.vertx.mutiny.sqlclient.Tuple;

import java.util.stream.StreamSupport;

public class Footballer {
  private Long id;
  private String firstName;
  private String lastName;
  private String position;

  public Footballer() {}

  public Footballer(Long id, String firstName, String lastName, String position) {
    this.id = id;
    this.firstName = firstName;
    this.lastName = lastName;
    this.position = position;
  }

  public Long getId() {
    return id;
  }

  public void setId(Long id) {
    this.id = id;
  }

  public String getFirstName() {
    return firstName;
  }

  public void setFirstName(String firstName) {
    this.firstName = firstName;
  }

  public String getLastName() {
    return lastName;
  }

  public void setLastName(String lastName) {
    this.lastName = lastName;
  }

  public String getPosition() {
    return position;
  }

  public void setPosition(String position) {
    this.position = position;
  }

  public static Uni<Footballer> create(Pool pool, Footballer footballer) {
    return pool.preparedQuery(
            "insert into footballer(first_name, last_name, position) values ($1, $2, $3) returning (id)",
            Tuple.of(footballer.firstName, footballer.lastName, footballer.position))
        .onItem()
        .apply(RowSet::iterator)
        .onItem()
        .apply(
            iterator ->
                new Footballer(
                    iterator.next().getLong("id"),
                    footballer.getFirstName(),
                    footballer.getLastName(),
                    footballer.getPosition()));
  }

  public static Multi<Footballer> findAll(PgPool pool) {
    return pool.query("select id, first_name, last_name, position from footballer")
        .onItem()
        .produceMulti(
            rowSet ->
                Multi.createFrom().items(() -> StreamSupport.stream(rowSet.spliterator(), false)))
        .onItem()
        .apply(Footballer::fromRow);
  }

  public static Multi<Footballer> findByPosition(PgPool pool, String position) {
    return pool.preparedQuery(
            "select id, first_name, last_name, position from footballer where position = $1",
            Tuple.of(position))
        .onItem()
        .produceMulti(
            rowSet ->
                Multi.createFrom().items(() -> StreamSupport.stream(rowSet.spliterator(), false)))
        .onItem()
        .apply(Footballer::fromRow);
  }

  public static Uni<Footballer> findOne(PgPool pool, Long id) {
    return pool.preparedQuery(
            "select id, first_name, last_name, position from footballer where id = $1",
            Tuple.of(id))
        .onItem()
        .apply(RowSet::iterator)
        .onItem()
        .apply(iterator -> iterator.hasNext() ? fromRow(iterator.next()) : null);
  }

  public static Uni<Boolean> delete(PgPool pool, Long id) {
    return pool.preparedQuery("delete from footballer where id = $1", Tuple.of(id))
        .onItem()
        .apply(rowSet -> rowSet.rowCount() == 1);
  }

  private static Footballer fromRow(Row row) {
    return new Footballer(
        row.getLong("id"),
        row.getString("first_name"),
        row.getString("last_name"),
        row.getString("position"));
  }
}
