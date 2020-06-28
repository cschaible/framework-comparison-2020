package info.novatec

import io.micronaut.http.HttpResponse
import io.micronaut.http.annotation.*
import java.util.*
import javax.transaction.Transactional

@Controller("/footballers")
open class FootballerRestController(
        private val repository: FootballerRepository
) {

    @Get("/")
    fun search(@QueryValue("position") position: String?): List<Footballer> {
        return if (position != null) {
            repository.findByPosition(position)
        } else {
            repository.findAll().toList()
        }
    }

    @Get("/{id}")
    fun get(id: Long): Optional<Footballer> {
        return repository.findById(id)
    }

    @Post("/")
    @Transactional
    open fun create(@Body footballer: Footballer): HttpResponse<Footballer> {
        return HttpResponse.created(repository.save(footballer))
    }

    @Delete("/{id}")
    @Transactional
    open fun delete(id: Long): HttpResponse<Footballer> {
        repository.deleteById(id)
        return HttpResponse.noContent<Footballer>()
    }
}