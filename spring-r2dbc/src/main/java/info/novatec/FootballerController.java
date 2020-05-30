package info.novatec;

import org.reactivestreams.Publisher;
import org.springframework.beans.factory.annotation.Autowired;
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
  public Mono<Footballer> findOne(@PathVariable Long id) {
    return footballerRepository.findById(id);
  }

  @PostMapping("/footballers")
  public Mono<Footballer> create(@RequestBody Publisher<Footballer> footballer) {
    return footballerRepository.create(footballer);
  }

  @DeleteMapping("/footballers/{id}")
  public Mono<Void> delete(@PathVariable Long id) {
    return footballerRepository.deleteById(id);
  }
}
