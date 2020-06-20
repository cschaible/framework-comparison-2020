import 'package:footballmanager/footballmanager.dart';

class Config extends Configuration {
  Config(String path) : super.fromFile(File(path));

  DatabaseConfiguration database;
}
