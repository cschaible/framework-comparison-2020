import 'package:footballmanager/footballer_resource.dart';
import 'package:footballmanager/footballmanager.dart';

@Table(name: "footballer")
class _Footballer {
  @primaryKey
  @Column(databaseType: ManagedPropertyType.bigInteger)
  int id;

  @Column(nullable: true)
  String first_name;

  @Column(nullable: true)
  String last_name;

  @Column(nullable: true)
  String position;
}

class Footballer extends ManagedObject<_Footballer> implements _Footballer {
  FootballerResource asResource() {
    final FootballerResource resource = FootballerResource();
    resource.id = id;
    resource.firstName = first_name;
    resource.lastName = last_name;
    resource.position = position;
    return resource;
  }
}
