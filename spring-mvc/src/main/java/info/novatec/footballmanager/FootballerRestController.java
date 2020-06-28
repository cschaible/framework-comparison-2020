package info.novatec.footballmanager;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.Optional;

@RequestMapping("/footballers")
@RestController
public class FootballerRestController {

  @Autowired private FootballerRepository footballerRepository;

  @GetMapping
  public List<Footballer> search(
      @RequestParam(name = "position", required = false) String position) {
    if (position == null) {
      return footballerRepository.findAll();
    } else {
      return footballerRepository.findByPosition(position);
    }
  }

  @GetMapping("/{id}")
  public ResponseEntity<Footballer> get(@PathVariable Long id) {
    Optional<Footballer> footballer = footballerRepository.findById(id);
    return footballer.map(ResponseEntity::ok).orElseGet(() -> ResponseEntity.notFound().build());
  }

  @Transactional
  @PostMapping
  public ResponseEntity<Footballer> create(@RequestBody Footballer footballer) {
    return ResponseEntity.status(HttpStatus.CREATED).body(footballerRepository.save(footballer));
  }

  @Transactional
  @DeleteMapping("/{id}")
  public ResponseEntity<Footballer> delete(@PathVariable Long id) {
    Optional<Footballer> footballer = footballerRepository.findById(id);
    footballer.ifPresent(value -> footballerRepository.delete(value));
    return ResponseEntity.noContent().build();
  }
}
