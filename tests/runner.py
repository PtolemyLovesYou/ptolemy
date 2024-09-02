"""Runner."""

import tvali


class TvaliTestRunner:
    """Tvali test runner."""

    VERSION = "0.0.1"
    ENV = "TEST"

    @classmethod
    def setup_method(cls):
        """Set up the test environment.

        This function is called before each test method to initialize the
        Tvali environment. It initializes the Tvali client with the
        version and environment specified in the class attributes.
        """
        tvali.init(client_type=tvali.ClientType.CONSOLE, version=cls.VERSION, environment=cls.ENV)

    @classmethod
    def teardown_method(cls):
        """Clean up the test environment.

        This function is called after each test method to clean up the
        Tvali environment. It resets the Tvali client to its initial state.
        """
        tvali.Config.initialized = False

    @property
    def client(self) -> tvali.Client:
        """Get the Tvali client.

        Returns the Tvali client initialized with the version and environment
        specified in the class attributes.
        """
        return tvali.client()
