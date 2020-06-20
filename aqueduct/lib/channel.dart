import 'package:footballmanager/config.dart';
import 'package:footballmanager/footballer_controller.dart';

import 'footballmanager.dart';

/// This type initializes an application.
///
/// Override methods in this class to set up routes and initialize services like
/// database connections. See http://aqueduct.io/docs/http/channel/.
class FootballmanagerChannel extends ApplicationChannel {
  ManagedContext context;

  /// Initialize services in this method.
  ///
  /// Implement this method to initialize services, read values from [options]
  /// and any other initialization required before constructing [entryPoint].
  ///
  /// This method is invoked prior to [entryPoint] being accessed.
  @override
  Future prepare() async {
    logger.onRecord.listen(
        (rec) => print("$rec ${rec.error ?? ""} ${rec.stackTrace ?? ""}"));
    logger.parent.level = Level.WARNING;
    final appOptions = Config(options.configurationFilePath);
    final dbOptions = appOptions.database;

    final dataModel = ManagedDataModel.fromCurrentMirrorSystem();
    final pps = PostgreSQLPersistentStore.fromConnectionInfo(
        dbOptions.username,
        dbOptions.password,
        dbOptions.host,
        dbOptions.port,
        dbOptions.databaseName);
    context = ManagedContext(dataModel, pps);
  }

  /// Construct the request channel.
  ///
  /// Return an instance of some [Controller] that will be the initial receiver
  /// of all [Request]s.
  ///
  /// This method is invoked after [prepare].
  @override
  Controller get entryPoint {
    final router = Router();

    router
        .route("/footballers/[:id]")
        .link(() => FootballerController(context));

    return router;
  }
}
