import 'package:footballmanager/footballer.dart';
import 'package:footballmanager/footballer_resource.dart';
import 'package:footballmanager/footballmanager.dart';

class FootballerController extends ResourceController {
  FootballerController(this.context);

  final ManagedContext context;

  @Operation.post()
  Future<Response> create(@Bind.body() FootballerResource resource) async {
    final query = Query<Footballer>(context)..values = resource.asFootballer();
    final savedFootballer = await query.insert();
    return Response.ok(savedFootballer.asResource());
  }

  @Operation.get()
  Future<Response> search({@Bind.query("position") String position}) async {
    if (position == null) {
      final query = Query<Footballer>(context);
      final footballers = await query.fetch();
      return Response.ok(await asResource(footballers));
    }
    final query = Query<Footballer>(context);
    query.where((footballer) => footballer.position).equalTo(position);
    final footballers = await query.fetch();
    return Response.ok(await asResource(footballers));
  }

  @Operation.get("id")
  Future<Response> findById(@Bind.path("id") int id) async {
    final query = Query<Footballer>(context);
    query.where((f) => f.id).equalTo(id);
    final footballer = await query.fetchOne();
    if (footballer == null) {
      return Response.notFound();
    }
    return Response.ok(footballer.asResource());
  }

  @Operation.delete("id")
  Future<Response> deleteById(@Bind.path("id") int id) async {
    final query = Query<Footballer>(context);
    query.where((footballer) => footballer.id).equalTo(id);
    await query.delete();
    return Response.noContent();
  }

  Future<List<FootballerResource>> asResource(
      List<Footballer> footballers) async {
    return Stream.fromIterable(footballers)
        .map((Footballer footballer) => footballer.asResource())
        .toList();
  }
}
