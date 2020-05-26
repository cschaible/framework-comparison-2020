package info.novatec.footballmanager;

import org.springframework.beans.factory.annotation.Autowired;
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
  public Optional<Footballer> get(@PathVariable Long id) {
    return footballerRepository.findById(id);
  }

  @PostMapping
  public Footballer create(@RequestBody Footballer footballer) {
    return footballerRepository.save(footballer);
  }

  @DeleteMapping("/{id}")
  public void delete(@PathVariable Long id) {
    footballerRepository.deleteById(id);
  }
}
