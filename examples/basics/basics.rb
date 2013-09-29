require 'resque'

class Basic
  @queue = :basic_queue
end

10.times do
  Resque.enqueue(Basic)
end
