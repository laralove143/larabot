defmodule Larabot.Schedule do
  @callback interval() :: any()
  @callback task() :: any()

  defmacro __using__(_) do
    quote do
      use GenServer
      @behaviour Larabot.Schedule

      def start_link(_) do
        GenServer.start_link(__MODULE__, nil, name: __MODULE__)
      end

      def init(_) do
        {:ok, nil}
      end

      def handle_cast(:task, _) do
        task()

        :timer.apply_after(interval(), &GenServer.cast/2, [__MODULE__, :task])

        {:noreply, nil}
      end

      def start do
        GenServer.cast(__MODULE__, :task)
      end
    end
  end
end
