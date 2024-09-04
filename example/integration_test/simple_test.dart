import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:frsr_flutter/frsr_flutter.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());
  test('Can call rust function', () async {
    expect(
        nextCard(
                currentMemoryState: null,
                daysElapsed: 0,
                status: CardStatus.hard)
            .interval,
        1);
  });
}
