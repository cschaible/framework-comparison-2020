package info.novatec;

import io.quarkus.test.junit.QuarkusTest;
import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;

@QuarkusTest
public class FootballerResourceTest {

  @Test
  public void testHelloEndpoint() {
    given().when().get("/footballers").then().statusCode(200);
  }
}
