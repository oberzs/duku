�      #     v                 GLSL.std.450                      main       .   1   5   9   R   _   d   f   l   o   q        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   modelspace_position      in_modelspace_position       worldspace_position      Constants            model_matrix            albedo_index            sampler_index        object       screenspace_position          Light             coords           color     %   WorldObject   %       world_matrix      %      lights    %      camera_position   %      time      %      light_matrices    %      cascade_splits    %      pcf   %      bias      '   world     .   out_modelspace_position   1   out_worldspace_position  	 5   out_screenspace_position      9   out_lightspace_position   R   out_normal    _   in_normal     d   out_uv    f   in_uv     j   gl_PerVertex      j       gl_Position   j      gl_PointSize      j      gl_ClipDistance   j      gl_CullDistance   l         o   out_color     q   in_color      s   MaterialObject    s       arg_1     s      arg_2     s      arg_3     s      arg_4     s      arg_5     s      arg_6     s      arg_7     s      arg_8     u   material    G            H            H         #       H               H        #   @   H        #   D   G        H          #       H         #      G  #          G  $      @   H  %          H  %       #       H  %             H  %      #   @   H  %      #   �   H  %      #   �   H  %         H  %      #   �   H  %            H  %      #   �  H  %      #   �  H  %      #   �  G  %      G  '   "       G  '   !       G  .         G  1         G  5         G  9         G  R          G  _         G  d         G  f         H  j              H  j            H  j            H  j            G  j      G  o         G  q         H  s       #       H  s      #      H  s      #       H  s      #   0   H  s      #   @   H  s      #   P   H  s      #   `   H  s      #   p   G  s      G  u   "      G  u   !            !                                          
                  
   ;           +          �?                                           	      ;        	   +                  	                    !           +  !   "        #       "     $      "    
 %      #   
      $               &      %   ;  &   '         (            -      
   ;  -   .      ;  -   1         4         ;  4   5        7      "      8      7   ;  8   9      +     :      +     @      +     F      +     L      ;  -   R        W   
      ;     _        b            c      b   ;  c   d         e      b   ;  e   f      +  !   h        i      h     j         i   i      k      j   ;  k   l      ;  4   o         p         ;  p   q       
 s                              t      s   ;  t   u      6               �     ;     	      ;           ;           =  
         Q               Q              Q              P                    >  	      A              =           =        	   �              >        A  (   )   '      =     *   )   =     +      �     ,   *   +   >     ,   =     /   	   O  
   0   /   /             >  .   0   =     2      O  
   3   2   2             >  1   3   =     6      >  5   6   A  (   ;   '   :      =     <   ;   =     =      �     >   <   =   A  4   ?   9      >  ?   >   A  (   A   '   :   @   =     B   A   =     C      �     D   B   C   A  4   E   9   @   >  E   D   A  (   G   '   :   F   =     H   G   =     I      �     J   H   I   A  4   K   9   F   >  K   J   A  (   M   '   :   L   =     N   M   =     O      �     P   N   O   A  4   Q   9   L   >  Q   P   A     S         =     T   S        U      "   T   T     V   U   Q     X   V       O  
   Y   X   X             Q     Z   V      O  
   [   Z   Z             Q     \   V      O  
   ]   \   \             P  W   ^   Y   [   ]   =  
   `   _   �  
   a   ^   `   >  R   a   =  b   g   f   >  d   g   =     m   	   A  4   n   l      >  n   m   =     r   q   >  o   r   �  8  �      #     Y                 GLSL.std.450                     main       ,   P   R   S   T   U   X                �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         fragment(        color_1      MaterialObject           arg_1           arg_2           arg_3           arg_4           arg_5           arg_6           arg_7           arg_8        material         color_2      percent      in_uv     %   color     ,   out_color     5   Light     5       coords    5      color     9   WorldObject   9       world_matrix      9      lights    9      camera_position   9      time      9      light_matrices    9      cascade_splits    9      pcf   9      bias      ;   world     <   Constants     <       model_matrix      <      albedo_index      <      sampler_index     >   object    C   textures      H   samplers      K   skybox    N   shadow_maps   P   in_normal     R   in_color      S   in_modelspace_position    T   in_worldspace_position    U   in_screenspace_position   X   in_lightspace_position  H         #       H        #      H        #       H        #   0   H        #   @   H        #   P   H        #   `   H        #   p   G        G     "      G     !       G           G  ,          H  5       #       H  5      #      G  7          G  8      @   H  9          H  9       #       H  9             H  9      #   @   H  9      #   �   H  9      #   �   H  9         H  9      #   �   H  9            H  9      #   �  H  9      #   �  H  9      #   �  G  9      G  ;   "       G  ;   !       H  <          H  <       #       H  <             H  <      #   @   H  <      #   D   G  <      G  C   "      G  C   !       G  H   "      G  H   !      G  K   "      G  K   !      G  N   "      G  N   !       G  P          G  R         G  S         G  T         G  U         G  X              !                   	            
      	               
                                        ;                       +                        +                                              ;                         +      !          "            +         ;  +   ,      +     .     �?  4           5         +      6        7   5   6     8   4   6    
 9   4   7   	      8               :      9   ;  :   ;        <   4            =   	   <   ;  =   >   	    	 ?                            +      @   d     A   ?   @      B       A   ;  B   C         D   +      E        F   D   E      G       F   ;  G   H        	 I                               J       I   ;  J   K         L   ?   6      M       L   ;  M   N          O      	   ;  O   P         Q         ;  Q   R      ;  O   S      ;  O   T      ;  Q   U        V      6      W      V   ;  W   X      6               �     9     3      �  8  6               �     ;  
         ;  
         ;           ;  
   %      A              =           O  	                      >        A              =           O  	                      >        A  "   #      !   =     $   #   >     $   =  	   &      =  	   '      =     (      P  	   )   (   (   (     	   *      .   &   '   )   >  %   *   =  	   -   %   Q     /   -       Q     0   -      Q     1   -      P     2   /   0   1   .   >  ,   2   �  8  