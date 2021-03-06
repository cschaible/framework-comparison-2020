import 'package:footballmanager/footballmanager.dart';

Future main() async {
  final app = Application<FootballmanagerChannel>()
      ..options.configurationFilePath = "config.src.yaml"
      ..options.port = 8080;

  final count = Platform.numberOfProcessors ~/ 2;
  await app.start(numberOfInstances: count > 0 ? count : 1);

  print("Application started on port: ${app.options.port}.");
  print("Use Ctrl-C (SIGINT) to stop running the application.");
}