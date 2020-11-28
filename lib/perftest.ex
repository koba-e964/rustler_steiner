# The functions in this module are for testing the performance of steiner_tree.ex
# Reference: https://rhye.org/post/native-scheduling-erlang/

defmodule PerfTest do
  def realtime_test() do
    n = 11
    edges = for i <- :lists.seq(0, n - 2), do: {i, i + 1}
    terms = :lists.seq(0, n - 1)

    for i <- :lists.seq(1, 10),
        do:
          spawn_link(fn ->
            realtime_worker(i, fn -> SteinerTree.compute(n, edges, terms) end)
          end)

    spawn_link(fn ->
      realtime_printer("yielding", :os.system_time())
    end)
  end

  def realtime_nonyielding_test() do
    n = 8
    edges = for i <- :lists.seq(0, n - 2), do: {i, i + 1}
    terms = :lists.seq(0, n - 1)

    for i <- :lists.seq(1, 10),
        do:
          spawn_link(fn ->
            realtime_worker(i, fn -> SteinerTree.compute_nonyielding(n, edges, terms) end)
          end)

    spawn_link(fn ->
      realtime_printer("nonyielding", :os.system_time())
    end)
  end

  def realtime_worker(index, fun) do
    fun.()

    IO.write([
      "[",
      Integer.to_string(index),
      "]"
    ])

    realtime_worker(index, fun)
  end

  def realtime_printer(name, last_run) do
    :timer.sleep(1000)
    delta = :os.system_time() - last_run
    delta_ms = delta / 1_000_000
    jitter_ms = 1000.0 - delta_ms

    IO.puts([
      "\n[",
      name,
      "]: Time since last schedule:",
      Float.to_string(delta_ms),
      " ms, ",
      "jitter: ",
      Float.to_string(jitter_ms),
      " ms"
    ])

    realtime_printer(name, :os.system_time())
  end
end
