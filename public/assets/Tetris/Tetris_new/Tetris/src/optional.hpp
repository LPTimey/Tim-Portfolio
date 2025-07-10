#pragma once

#include <assert.h>
#include <new> // placement new

// === Hilfs-Typ: AlignedStorage ===
template <typename T> struct AlignedStorage {
    alignas(T) char data[sizeof(T)];
    T* get() { return reinterpret_cast<T*>(&data); }
    const T* get() const { return reinterpret_cast<const T*>(&data); }
};

// === Hilfsfunktionen: move, forward (rudimentär) ===
template <typename T> T&& move(T& t) { return static_cast<T&&>(t); }

template <typename T> T&& forward(T& t) { return static_cast<T&&>(t); }

// === nullopt ===
struct nullopt_t {
    explicit constexpr nullopt_t(int) {}
};
constexpr nullopt_t nullopt{0};

// === Option<T> ===
template <typename T> class Option {
    bool has_value_;
    AlignedStorage<T> storage_;

    T* ptr() { return storage_.get(); }
    const T* ptr() const { return storage_.get(); }

  public:
    Option() : has_value_(false) {}
    Option(nullopt_t) : has_value_(false) {}

    Option(const Option& other) : has_value_(other.has_value_) {
        if (has_value_) {
            new (ptr()) T(*other);
        }
    }

    Option(Option&& other) : has_value_(other.has_value_) {
        if (has_value_) {
            new (ptr()) T(move(*other));
        }
    }

    Option(const T& value) : has_value_(true) { new (ptr()) T(value); }

    Option(T&& value) : has_value_(true) { new (ptr()) T(move(value)); }

    ~Option() { reset(); }

    Option& operator=(const T& value) {
        if (has_value_) {
            **this = value;
        } else {
            new (ptr()) T(value);
            has_value_ = true;
        }
        return *this;
    }

    Option& operator=(T&& value) {
        if (has_value_) {
            **this = move(value);
        } else {
            new (ptr()) T(move(value));
            has_value_ = true;
        }
        return *this;
    }

    Option& operator=(const Option& other) {
        if (this != &other) {
            if (other.has_value_) {
                *this = *other;
            } else {
                reset();
            }
        }
        return *this;
    }

    Option& operator=(Option&& other) {
        if (this != &other) {
            if (other.has_value_) {
                *this = move(*other);
            } else {
                reset();
            }
        }
        return *this;
    }

    Option& operator=(nullopt_t) {
        reset();
        return *this;
    }

    T& operator*() { return *ptr(); }
    const T& operator*() const { return *ptr(); }

    T* operator->() { return ptr(); }
    const T* operator->() const { return ptr(); }

    bool has_value() const { return has_value_; }
    explicit operator bool() const { return has_value_; }

    void reset() {
        if (has_value_) {
            ptr()->~T();
            has_value_ = false;
        }
    }

    T value_or(const T& default_value) const {
        return has_value_ ? **this : default_value;
    }

    T& value() {
        assert(has_value_);
        return **this;
    }

    const T& value() const {
        assert(has_value_);
        return **this;
    }

    template <typename... Args> T& emplace(Args&&... args) {
        reset();
        new (ptr()) T(forward<Args>(args)...);
        has_value_ = true;
        return **this;
    }

    bool operator==(nullopt_t) const { return !has_value_; }
    bool operator!=(nullopt_t) const { return has_value_; }
};
