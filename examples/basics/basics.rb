require 'resque'

class Basic
  @queue = :basic_queue
end

3.times do
  Resque.enqueue(Basic)
end
