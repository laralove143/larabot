defmodule Larabot.Consumer do
  @callback handle(any()) :: any()
end
