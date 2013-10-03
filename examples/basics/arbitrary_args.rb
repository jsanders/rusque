require 'resque'

class Basic
  @queue = :basic_queue
end

Resque.enqueue(Basic, 'some', 'args', 1, [ 2, 3 ], { w: :x, :y => 'z' })
