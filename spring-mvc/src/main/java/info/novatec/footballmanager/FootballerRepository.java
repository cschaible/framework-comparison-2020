package info.novatec.footballmanager;

import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

public interface FootballerRepository extends JpaRepository<Footballer, Long> {

  List<Footballer> findByPosition(String position);
}
