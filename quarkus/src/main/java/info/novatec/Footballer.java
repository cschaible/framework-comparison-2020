package info.novatec;

import io.quarkus.hibernate.orm.panache.PanacheEntity;
import io.quarkus.hibernate.orm.panache.PanacheQuery;

import javax.persistence.Entity;

@Entity
public class Footballer extends PanacheEntity {

  public String firstName;

  public String lastName;

  public String position;

  public static PanacheQuery<Footballer> findByPosition(String position) {
    return find("position", position);
  }
}
