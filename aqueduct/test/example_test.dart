import 'harness/app.dart';

Future main() async {
  final harness = Harness()..install();

  test("GET /footballers returns 200", () async {
    expectResponse(await harness.agent.get("/footballers"), 200);
  });
}
