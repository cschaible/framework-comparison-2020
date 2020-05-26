package info.novatec.footballmanager;

import org.springframework.data.jpa.domain.AbstractPersistable;

import javax.persistence.Entity;

@Entity
public class Footballer extends AbstractPersistable<Long> {

  public String firstName;

  public String lastName;

  public String position;
}
