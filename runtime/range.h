#pragma once

class iterator {
    private:
        int idx_;
    public:
        iterator(int idx) : idx_(idx) {};
        int operator *() const { return idx_; };
        const iterator &operator ++() { idx_++; return *this; };
        iterator operator ++(int) { 
        iterator iter = *this;
        //Iterator iter(*this);
        idx_++; 
        return iter; 
        };
        bool operator !=(const iterator &iter) { return idx_ != iter.idx_; };
};


class range {
    private:
        iterator begin_;
        iterator end_;
    public:
        range(int end) : begin_(0), end_(end) {};
        iterator begin() const { return begin_; };
        iterator end() const { return end_; };
};
