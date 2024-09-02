"""Runner."""
import tvali

class TvaliTestRunner:
    """Tvali test runner."""
    VERSION = '0.0.1'
    ENV = 'TEST'

    @classmethod
    def setup_method(cls):
        tvali.init(
            client_type=tvali.ClientType.CONSOLE,
            version=cls.VERSION,
            environment=cls.ENV
            )

    @classmethod
    def teardown_method(cls):
        tvali.Config.initialized = False

    @property
    def client(self) -> tvali.Client:
        return tvali.client()
