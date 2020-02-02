# Experiment for operation time on different variable type (owner, reference, slice)

## Description
In this experiment, the goal is to assess how efficiency of operation among different variable types are different. 
Question asked in this experiment is whether different way of memory allocation for each variable type effects rum time performance. To measure the time of operation, three objects are implemented. Each object has different type of variable for its fields. CustomerOwed object has fields whose varirable type are owner of String. CustomerBorrowed has fielss whose variable type are reference to String. CustomerSlice has fields whose variable type are slice of string.

The detail design of experiment is following. First, vectors of random String whose length is 5 is created. Then, passing the String elements to each Objects to create them. These objects are stored in vectors of unique object type. For example of CustomerOwned, we create Vec<CustomerOwned>. After creation of object vectors, each elements of vector and thier fields (owner, reference, slice of String) are accessed. 

The operation when accessing fields of object is performed through serializing and deserializing the object. This de/serialization is handcoded. For serialization, bytes of each field String and the lenghts are obtained and stored in bytes array. Vice versa for deserialization. We measure operation time for 1000000, 1500000, 10000000, 15000000 of each objects, CustomerOwned, CustomerBorrowed, and CustomerSlice. 

The result shows that access times to onwer and reference are almost the same, but the one to slice is relatively faster compared to the other two.



```
struct CustomerOwned {
    zip_code: String,
    address: String,
    country: String
}

struct CustomerBorrowed<'a> {
    zip_code: &'a String,
    address: &'a String,
    country: &'a String
}

struct CustomerSlice<'a> {
    zip_code: &'a str,
    address: &'a str,
    country: &'a str
}
```

![Image description](https://github.com/ShinsakuOkazaki/master-thesis/tree/master/project/rust_object_ref_own_slice/analysis/rust_borrowing.png)