```java 

public class Main {
    
    static class Obj {
        String name;
        public Obj(String name){
            this.name = name;
        }
        public void setName(String name){
            this.name = name;
        }
        public String getName(){
            return name;
        }
    }
	public static void main(String[] args) {
		Obj a = new Obj("good");
		Obj c;
		
		{
		    Obj b =  new Obj("bad");
		    c = cmp(a,b);
		}
		
		System.out.println(c.getName());
	}
	
	private static Obj cmp(Obj a,Obj b) {
	    if(a.getName().length() < b.getName().length()){
	        return a;
	    }else{
	        return b;
	    }
	}
}


```