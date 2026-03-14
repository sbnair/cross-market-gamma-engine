#pragma once
#include <array>
#include <atomic>
#include <cstddef>

template <typename T, std::size_t N>
class SpscRing {
public:
    bool push(const T& item) {
        const auto head = head_.load(std::memory_order_relaxed);
        const auto next = (head + 1) % N;
        if (next == tail_.load(std::memory_order_acquire)) return false;
        buffer_[head] = item;
        head_.store(next, std::memory_order_release);
        return true;
    }

    bool pop(T& out) {
        const auto tail = tail_.load(std::memory_order_relaxed);
        if (tail == head_.load(std::memory_order_acquire)) return false;
        out = buffer_[tail];
        tail_.store((tail + 1) % N, std::memory_order_release);
        return true;
    }

private:
    std::array<T, N> buffer_{};
    std::atomic<std::size_t> head_{0};
    std::atomic<std::size_t> tail_{0};
};
