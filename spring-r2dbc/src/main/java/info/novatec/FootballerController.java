package info.novatec;

import org.reactivestreams.Publisher;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

@RestController
public class FootballerController {

  @Autowired private FootballerRepository footballerRepository;

  @GetMapping("/footballers")
  public Flux<Footballer> search(
      @RequestParam(name = "position", required = false) String position) {
    if (position != null) {
      return footballerRepository.findByPosition(position);
    } else {
      return footballerRepository.findAll();
    }
  }

  @GetMapping("/footballers/{id}")
  public Mono<ResponseEntity<Footballer>> findOne(@PathVariable Long id) {
    return footballerRepository
        .findById(id)
        .map(ResponseEntity::ok)
        .switchIfEmpty(Mono.just(ResponseEntity.status(HttpStatus.NOT_FOUND).build()));
  }

  @PostMapping("/footballers")
  public ResponseEntity<Mono<Footballer>> create(@RequestBody Publisher<Footballer> footballer) {
    return ResponseEntity.status(HttpStatus.CREATED).body(footballerRepository.create(footballer));
  }

  @DeleteMapping("/footballers/{id}")
  public ResponseEntity<Mono<Void>> delete(@PathVariable Long id) {
    return ResponseEntity.status(HttpStatus.NO_CONTENT).body(footballerRepository.deleteById(id));
  }
}
