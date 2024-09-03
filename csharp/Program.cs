using System.Diagnostics;
using LList;
using Tree;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var co = new Codec();
    var repr = "[1,2,3,null,null,4,5]";
    var n = Codec.Deserialize(repr);
    var s = Codec.Serialize(n);
    Debug.Assert(s == repr);
}

void Test2()
{
    var co = new Codec();
    var repr = "[]";
    var n = Codec.Deserialize(repr);
    var s = Codec.Serialize(n);
    Debug.Assert(s == repr);
}
