import 'package:footballmanager/footballer.dart';

import 'footballmanager.dart';

class FootballerResource extends Serializable {
  int id;

  String firstName;

  String lastName;

  String position;

  Footballer asFootballer() {
    final footballer = Footballer();
    footballer.first_name = firstName;
    footballer.last_name = lastName;
    footballer.position = position;
    return footballer;
  }

  @override
  Map<String, dynamic> asMap() {
    return {
      "id": id,
      "firstName": firstName,
      "lastName": lastName,
      "position": position
    };
  }

  @override
  void readFromMap(Map<String, dynamic> inputMap) {
    firstName = inputMap["firstName"] as String;
    lastName = inputMap["lastName"] as String;
    position = inputMap["position"] as String;
  }
}
