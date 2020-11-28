defmodule RustlerSteinerTest do
  use ExUnit.Case
  doctest RustlerSteiner

  test "greets the world" do
    assert RustlerSteiner.hello() == :world
  end

  test "realtime_test" do
    PerfTest.realtime_test()
    :timer.sleep(15000)
    assert true
  end

  test "realtime_nonyielding_test" do
    PerfTest.realtime_nonyielding_test()
    :timer.sleep(15000)
    assert true
  end
end
