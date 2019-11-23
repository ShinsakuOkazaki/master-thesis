/*
 * Copyright (c) 2014, Oracle America, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *  * Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 *  * Neither the name of Oracle nor the names of its contributors may be used
 *    to endorse or promote products derived from this software without
 *    specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

package com.test;

import org.openjdk.jmh.annotations.*;

import java.util.*;
import java.util.concurrent.TimeUnit;

public class MyBenchmark {

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test9vs10() {
        // Set the parameters to test.
        int initial_size = 10;
        int final_size = 9;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test9vs100() {
        // Set the parameters to test.
        int initial_size = 100;
        int final_size = 9;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test9vs1000() {
        // Set the parameters to test.
        int initial_size = 1000;
        int final_size = 9;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test9vs10000() {
        // Set the parameters to test.
        int initial_size = 10000;
        int final_size = 9;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }
    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test99vs10() {
        // Set the parameters to test.
        int initial_size = 10;
        int final_size = 99;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test99vs100() {
        // Set the parameters to test.
        int initial_size = 100;
        int final_size = 99;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test99vs1000() {
        // Set the parameters to test.
        int initial_size = 1000;
        int final_size = 99;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test99vs10000() {
        // Set the parameters to test.
        int initial_size = 10000;
        int final_size = 99;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test999vs10() {
        // Set the parameters to test.
        int initial_size = 10;
        int final_size = 999;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test999vs100() {
        // Set the parameters to test.
        int initial_size = 100;
        int final_size = 999;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test999vs1000() {
        // Set the parameters to test.
        int initial_size = 1000;
        int final_size = 999;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test999vs10000() {
        // Set the parameters to test.
        int initial_size = 10000;
        int final_size = 999;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test9999vs10() {
        // Set the parameters to test.
        int initial_size = 2;
        int final_size = 9999;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test9999vs100() {
        // Set the parameters to test.
        int initial_size = 100;
        int final_size = 9999;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test9999vs1000() {
        // Set the parameters to test.
        int initial_size = 1000;
        int final_size = 9999;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }

    @Benchmark
    @BenchmarkMode(Mode.AverageTime)
    @OutputTimeUnit(TimeUnit.NANOSECONDS)
    @Warmup(iterations = 2, time = 1, timeUnit = TimeUnit.SECONDS)
    @Threads(value = 1)
    @Fork(value = 2, jvmArgs = {"-Xms2G", "-Xmx2G"})
    public void test9999vs10000() {
        // Set the parameters to test.
        int initial_size = 10000;
        int final_size = 9999;

        // Allocate ArrayList initializing its size to big.
        List<Integer> arrayList = new ArrayList<Integer>(initial_size);

        // Add to the ArrayList
        for (int i = 0; i < final_size; i ++) {
            arrayList.add(i);
        }
    }
}
